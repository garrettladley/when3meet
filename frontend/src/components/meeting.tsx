import { createSignal, onMount } from 'solid-js';
import { useLocation } from 'solid-start';
import { API_BASE_URL } from '~/consts';
import { Meeting as MeetingT } from '~/main.types';

interface MeetingProps {
  id: string;
}

export default function Meeting(props: MeetingProps) {
  const location = useLocation();

  const [meetingData, setMeetingData] = createSignal<MeetingT>();

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
    if (location.state) {
      setMeetingData(location.state as MeetingT);
    } else {
      fetchMeetingData();
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
