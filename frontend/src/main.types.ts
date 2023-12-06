export interface Slot {
  start: Date;
  end: Date;
}

export interface User {
  id: string;
  name: string;
  availability: Slot[];
}

export interface Timestamp24Hr {
  hour: number;
  minute: number;
}

export interface Meeting {
  id: string;
  start: Date;
  end: Date;
  noEarlierThan: Timestamp24Hr;
  noLaterThan: Timestamp24Hr;
  users: User[];
}
