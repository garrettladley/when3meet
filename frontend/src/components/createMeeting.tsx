import { createSignal } from "solid-js";

const getDefaultDate = () => {
  const date = new Date();
  date.setHours(0, 0, 0, 0);
  return date.toISOString().split("T")[0];
};

const getDefaultEndDate = () => {
  const startDate = new Date();
  startDate.setDate(startDate.getDate() + ((5 - startDate.getDay() + 7) % 7));
  return startDate.toISOString().split("T")[0];
};

const getDefaultTime = (hours: number, minutes: number): string => {
  const formattedHours = hours < 10 ? `0${hours}` : `${hours}`;
  const formattedMinutes = minutes < 10 ? `0${minutes}` : `${minutes}`;

  return `${formattedHours}:${formattedMinutes}`;
};

const CreateMeeting = () => {
  const [meetingName, setMeetingName] = createSignal<string>("");
  const [startDate, setStartDate] = createSignal<string>(getDefaultDate());
  const [endDate, setEndDate] = createSignal<string>(getDefaultEndDate());
  const [noEarlierThan, setNoEarlierThan] = createSignal<string>(
    getDefaultTime(9, 0)
  );
  const [noLaterThan, setNoLaterThan] = createSignal<string>(
    getDefaultTime(17, 0)
  );

  const isValidDateRange = new Date(startDate()) < new Date(endDate());
  const isNoEarlierLaterThanValid = () => {
    const noEarlierThanHour = parseInt(noEarlierThan().split(":")[0]);
    const noEarlierThanMinute = parseInt(noEarlierThan().split(":")[1]);
    const noLaterThanHour = parseInt(noLaterThan().split(":")[0]);
    const noLaterThanMinute = parseInt(noLaterThan().split(":")[1]);

    return (
      noEarlierThanHour < noLaterThanHour ||
      (noEarlierThanHour === noLaterThanHour &&
        noEarlierThanMinute < noLaterThanMinute)
    );
  };
  const isFormValid = () => {
    return (
      meetingName() !== "" && isValidDateRange && isNoEarlierLaterThanValid()
    );
  };

  const createMeeting = async () => {
    try {
      const response = await fetch("http://127.0.0.1:8000/meeting/create", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        mode: "cors",
        credentials: "same-origin",
        body: JSON.stringify({
          name: meetingName(),
          start_date: new Date(startDate()).toISOString(),
          end_date: new Date(endDate()).toISOString(),
          no_earlier_than_hr: parseInt(noEarlierThan().split(":")[0]),
          no_earlier_than_min: parseInt(noEarlierThan().split(":")[1]),
          no_later_than_hr: parseInt(noLaterThan().split(":")[0]),
          no_later_than_min: parseInt(noLaterThan().split(":")[1]),
        }),
      });

      if (response.ok) {
        const meetingId = await response.text();
        console.log(
          `Meeting created successfully. Redirecting to /meeting/${meetingId}`
        );
      } else {
        console.error("Failed to create meeting");
      }
    } catch (error) {
      console.error("Error creating meeting", error);
    }
  };

  return (
    <div>
      <h1 class="text-3xl font-bold">Create a Meeting</h1>
      <div class="mt-4">
        <input
          class="shadow border rounded w-1/4 py-2 px-3 text-gray-700 leading-tight"
          type="text"
          placeholder="Meeting Name"
          value={meetingName()}
          onChange={(e) => setMeetingName(e.target.value)}
        />
        <div class="mt-4">
          <label class="py-2 px-3" for="startDate">
            Start Date
          </label>
          <input
            class="shadow border  rounded py-2 px-3 text-gray-700 leading-tight"
            type="date"
            id="startDate"
            value={startDate()}
            onChange={(e) => setStartDate(e.target.value)}
          />
        </div>
        <div class="mt-4">
          <label class="py-2 px-3" for="endDate">
            End Date
          </label>
          <input
            class="shadow border  rounded py-2 px-3 text-gray-700 leading-tight"
            type="date"
            id="endDate"
            value={endDate()}
            onChange={(e) => setEndDate(e.target.value)}
          />
        </div>
        <div class="mt-4">
          <label class="py-2 px-3" for="noEarlierThan">
            No Earlier Than
          </label>
          <input
            class="shadow border  rounded py-2 px-3 text-gray-700 leading-tight"
            type="time"
            id="noEarlierThan"
            value={noEarlierThan()}
            onChange={(e) => setNoEarlierThan(e.target.value)}
          />
        </div>
        <div class="mt-4">
          <label class="py-2 px-3" for="noLaterThan">
            No Later Than
          </label>
          <input
            class="shadow border  rounded py-2 px-3 text-gray-700 leading-tight"
            type="time"
            id="noLaterThan"
            value={noLaterThan()}
            onChange={(e) => setNoLaterThan(e.target.value)}
          />
        </div>
        <div class="mt-4">
          <button
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            onClick={() => {
              if (isFormValid()) {
                createMeeting();
              } else {
                console.error("Form is invalid");
              }
            }}
          >
            Create
          </button>
        </div>
      </div>
    </div>
  );
};

export default CreateMeeting;
