export type User = {
  id: number;
  name: string;
  profilePicture?: string;
  token?: {
    token?: string;
  };
};

export type UserDisplayPartial = Pick<User, "id" | "name" | "profilePicture">;

export enum Status {
  CONNECTED = "connected",
  AWAY = "away",
  CONNECTING = "connecting",
  DISCONNECTED = "disconnected",
}
