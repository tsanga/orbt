"use client";

import { createContext, Dispatch, useReducer, useEffect } from "react";
import { User } from "@domain/models";
export const AppContext = createContext<State>({ user: null });
export const AppDispatchContext = createContext<Dispatch<Action>>(() => {});

type Props = {
  children: React.ReactNode;
};

export type State = {
  user: Partial<User> | null;
};

export type Action = { type: "setUser"; user: Partial<User> };

const reducer = (state: State, action: Action): State => {
  console.log("hi");
  switch (action.type) {
    case "setUser":
      console.log("set");
      return { ...state, user: action.user };
  }
};

export default function AppContextWrapper({ children }: Props) {
  const [state, dispatch] = useReducer(reducer, { user: null });

  return (
    <AppContext.Provider value={state}>
      <AppDispatchContext.Provider value={dispatch}>
        {children}
      </AppDispatchContext.Provider>
    </AppContext.Provider>
  );
}
