"use client";

import { createContext, useReducer } from "react";

export const RoomContext = createContext(null);
export const RoomDispatchContext = createContext(null);
/*
type Props = {
  children: React.ReactNode;
};

type State = {
    user:
}

const reducer = (state, action) => {

}

export default function RoomContextWrapper({ children }: Props) {
  const [tasks, dispatch] = useReducer();

  <RoomContext.Provider>
    <RoomDispatchContext.Provider>{children}</RoomDispatchContext.Provider>
  </RoomContext.Provider>;
}
*/
