import {
  RoomContext,
  RoomDispatchContext,
  State,
  Action,
} from "@/room/context";
import { useContext, Dispatch } from "react";

export default function useRoomContext(): [State, Dispatch<Action>] {
  const state = useContext(RoomContext);
  const dispatch = useContext(RoomDispatchContext);

  return [state!!, dispatch!!];
}
