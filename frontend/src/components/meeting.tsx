import { createSignal, onMount } from "solid-js";

import { API_BASE_URL } from "../consts";
import { Meeting as MeetingType } from "../main.types";

interface MeetingProps {
  meetingId: string;
}

export default function Meeting(props: MeetingProps) {
  const [meetingData, setMeetingData] = createSignal<MeetingType>();

  const fetchMeetingData = async () => {
    try {
      const response = await fetch(
        `${API_BASE_URL}/meeting/${props.meetingId}`,
        {
          method: "GET",
          mode: "cors",
          credentials: "same-origin",
        }
      );
      const data = await response.json();
      setMeetingData(data);
    } catch (error) {
      console.error("Error fetching meeting data:", error);
    }
  };

  onMount(() => {
    fetchMeetingData();
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
