import { useParams } from "solid-start";
import Meeting from "~/components/meeting";

export default function MeetingDashboard() {
  const params = useParams();

  return <Meeting id={params.id} />;
}
