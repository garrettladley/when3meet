export interface DBMeeting {
  id: string;
  meeting: UserMeeting;
}

export interface UserMeeting {
  meeting: Meeting;
  users: User[];
}

export interface Meeting {
  start: Date;
  end: Date;
  noEarlierThan: Timestamp24Hr;
  noLaterThan: Timestamp24Hr;
}

export interface Timestamp24Hr {
  hour: number;
  minute: number;
}

export interface Slot {
  start: Date;
  end: Date;
}

export interface User {
  id: string;
  name: string;
  availability: Slot[];
}
