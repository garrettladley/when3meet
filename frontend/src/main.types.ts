export interface DBMeeting {
  id: string;
  meeting: Meeting;
}

export interface Meeting {
  name: string;
  range: TimeRange;
  users: User[];
}

export interface TimeRange {
  start: Date;
  end: Date;
}

export interface User {
  id: string;
  name: string;
  availability: TimeRange[];
}
