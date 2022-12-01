export type User = {
  id: string;
  name: string;
  status?: string;
  profilePicture?: string;
  token?: {
    token?: string;
  };
};

export type UserDisplayPartial = Pick<
  User,
  "id" | "name" | "profilePicture" | "status"
>;

export enum Status {
  CONNECTED = "connected",
  AWAY = "away",
  CONNECTING = "connecting",
  DISCONNECTED = "disconnected",
}
