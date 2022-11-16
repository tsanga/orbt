export type User = {
  name: string;
  profilePicture: string;
  status: Status;
};

export type UserDisplayPartial = Pick<
  User,
  "name" | "profilePicture" | "status"
>;

export enum Status {
  CONNECTED = "connected",
  AWAY = "away",
  CONNECTING = "connecting",
  DISCONNECTED = "disconnected",
}
