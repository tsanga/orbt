"use client";

import { createContext, Dispatch, useReducer } from "react";

export const RoomContext = createContext<State | null>(null);
export const RoomDispatchContext = createContext<Dispatch<Action> | null>(null);

type Props = {
  children: React.ReactNode;
};

type State = {
  isCreatingRoom: boolean;
  isJoiningRoom: boolean;
  room?: string | null | undefined;
};

type Action =
  | { type: "SET_IS_CREATING_ROOM"; isCreatingRoom: boolean }
  | { type: "SET_IS_JOINING_ROOM"; isJoiningRoom: boolean };

const reducer = (state: State, action: Action) => {
  switch (action.type) {
    case "SET_IS_CREATING_ROOM": {
      return { ...state, isCreatingRoom: action.isCreatingRoom };
    }
    case "SET_IS_JOINING_ROOM": {
      return { ...state, isJoiningRoom: action.isJoiningRoom };
    }
  }
};

export default function RoomContextWrapper({ children }: Props) {
  const [tasks, dispatch] = useReducer(reducer, {
    isCreatingRoom: false,
    isJoiningRoom: false,
  });

  return (
    <RoomContext.Provider value={tasks}>
      <RoomDispatchContext.Provider value={dispatch}>
        {children}
      </RoomDispatchContext.Provider>
    </RoomContext.Provider>
  );
}
