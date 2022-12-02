"use client";

import { Room } from "@domain/models";
import { createContext, Dispatch, useReducer } from "react";

export const RoomContext = createContext<State | null>(null);
export const RoomDispatchContext = createContext<Dispatch<Action> | null>(null);

type Props = {
  children: React.ReactNode;
};

export type State = {
  isCreatingRoom: boolean;
  isJoiningRoom: boolean;
  room?: Partial<Room> | null | undefined;
  generatedInviteCode?: string;
};

export type Action =
  | { type: "SET_IS_CREATING_ROOM"; isCreatingRoom: boolean }
  | { type: "SET_IS_JOINING_ROOM"; isJoiningRoom: boolean }
  | { type: "SET_ROOM"; room: Partial<Room> }
  | { type: "SET_GENERATED_INVITE_CODE"; generatedInviteCode: string };

const initializer = (): State => {
  if (typeof window === "undefined")
    return { isCreatingRoom: true, isJoiningRoom: false };

  let room = localStorage.getItem("room");

  if (room) {
    return JSON.parse(room);
  }

  return {
    isCreatingRoom: true,
    isJoiningRoom: false,
  };
};

const reducer = (state: State, action: Action) => {
  let toReturn;

  switch (action.type) {
    case "SET_IS_CREATING_ROOM": {
      toReturn = { ...state, isCreatingRoom: action.isCreatingRoom };
      break;
    }
    case "SET_IS_JOINING_ROOM": {
      toReturn = { ...state, isJoiningRoom: action.isJoiningRoom };
      break;
    }
    case "SET_ROOM": {
      toReturn = { ...state, room: action.room };
      break;
    }
    case "SET_GENERATED_INVITE_CODE": {
      toReturn = { ...state, generatedInviteCode: action.generatedInviteCode };
      break;
    }
  }

  if (typeof window !== "undefined") {
    localStorage.setItem("room", JSON.stringify(toReturn));
  }

  return toReturn;
};

export default function RoomContextWrapper({ children }: Props) {
  const [tasks, dispatch] = useReducer(reducer, {}, initializer);

  return (
    <RoomContext.Provider value={tasks}>
      <RoomDispatchContext.Provider value={dispatch}>
        {children}
      </RoomDispatchContext.Provider>
    </RoomContext.Provider>
  );
}
