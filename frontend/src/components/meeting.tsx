import { createSignal, onMount } from "solid-js";
import { API_BASE_URL } from "~/consts";
import { Meeting } from "~/main.types";

interface MeetingProps {
  new?: boolean;
  id: string;
  meeting?: Meeting;
}

export default function Meeting(props: MeetingProps) {
  const [meetingData, setMeetingData] = createSignal<Meeting>();

  const fetchMeetingData = async () => {
    try {
      const response = await fetch(`${API_BASE_URL}/meeting/${props.id}`, {
        method: "GET",
        mode: "cors",
        credentials: "same-origin",
      });
      const data = await response.json();
      setMeetingData(data);
    } catch (error) {
      console.error("Error fetching meeting data:", error);
    }
  };

  onMount(() => {
    if (!props.meeting) {
      fetchMeetingData();
    } else {
      setMeetingData(props.meeting);
    }
  });

  return (
    <div>
      {meetingData() ? (
        JSON.stringify(meetingData())
      ) : (
        <p>Loading meeting data...</p>
      )}
    </div>
  );
}
