import { createSignal } from 'solid-js';
import { API_BASE_URL } from '~/consts';
import { Meeting as MeetingT, TimeRange } from '~/main.types';

import { useNavigate } from '@solidjs/router';

const getDefaultDate = () => {
  const date = new Date();
  date.setHours(9);
  date.setMinutes(0);
  date.setSeconds(0);
  date.setMilliseconds(0);
  return date;
};

const getDefaultEndDate = () => {
  const startDate = getDefaultDate();
  startDate.setDate(startDate.getDate() + ((5 - startDate.getDay() + 7) % 7));
  startDate.setHours(17);
  startDate.setMinutes(0);
  startDate.setSeconds(0);
  startDate.setMilliseconds(0);
  return startDate;
};

const CreateMeeting = () => {
  const navigate = useNavigate();

  const [meetingName, setMeetingName] = createSignal<string>("");
  const [startRange, setStartRange] = createSignal<Date>(getDefaultDate());
  const [endRange, setEndRange] = createSignal<Date>(getDefaultEndDate());

  const isFormValid = () => {
    return (
      meetingName() !== "" &&
      startRange() < endRange() &&
      startRange().getTime() < endRange().getTime()
    );
  };

  const toDateString = (date: Date) => {
    return date.toISOString().split("T")[0];
  };

  const toTimeString = (date: Date) => {
    return `${String(date.getHours()).padStart(2, "0")}:${String(
      date.getMinutes()
    ).padStart(2, "0")}`;
  };

  const timeStringSetDate = (date: Date, timeString: string) => {
    const [hours, minutes] = timeString.split(":");
    date.setHours(Number(hours));
    date.setMinutes(Number(minutes));
    return date;
  };

  const createMeeting = async () => {
    try {
      const response = await fetch(`${API_BASE_URL}/meeting/create`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        mode: "cors",
        credentials: "same-origin",
        body: JSON.stringify({
          name: meetingName(),
          start: startRange().toISOString(),
          end: endRange().toISOString(),
        }),
      });

      if (response.ok) {
        const meetingId = await response.text();
        navigate(`/${meetingId}`, {
          state: {
            name: meetingName(),
            range: {
              start: startRange(),
              end: endRange(),
            } as TimeRange,
            users: [],
          } as MeetingT,
        });
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
            value={toDateString(startRange())}
            onChange={(e) => setStartRange(new Date(e.target.value))}
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
            value={toDateString(endRange())}
            onChange={(e) => setEndRange(new Date(e.target.value))}
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
            value={toTimeString(startRange())}
            onChange={(e) =>
              setStartRange(timeStringSetDate(startRange(), e.target.value))
            }
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
            value={toTimeString(endRange())}
            onChange={(e) =>
              setEndRange(timeStringSetDate(endRange(), e.target.value))
            }
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
