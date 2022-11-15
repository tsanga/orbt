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
  Connecting = "CONNECTING",
  Connected = "CONNECTED",
  Away = "AWAY",
  Disconnected = "DISCONNECTED",
}
