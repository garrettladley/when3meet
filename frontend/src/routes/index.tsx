import { A } from "solid-start";
import CreateMeeting from "~/components/createMeeting";

export default function Home() {
  return (
    <>
      <main class="text-center mx-auto text-gray-700 p-4">
        <h1 class="max-6-xs text-6xl text-sky-700 font-thin uppercase my-16">
          Welcome to when3meet!
        </h1>
        <CreateMeeting />
      </main>
    </>
  );
}
