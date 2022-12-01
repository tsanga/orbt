"use client";

import { createContext, Dispatch, useReducer, useEffect } from "react";
import { User } from "@domain/models";
export const AppContext = createContext<State>({
  user: null,
});
export const AppDispatchContext = createContext<Dispatch<Action>>(() => {});

export type AuthUser = Pick<User, "name" | "id" | "token">;

type Props = {
  children: React.ReactNode;
};

export type State = {
  user: AuthUser | null;
};

export type Action = { type: "SET_USER"; user: AuthUser | null };

const reducer = (state: State, action: Action): State => {
  switch (action.type) {
    case "SET_USER":
      return { ...state, user: action.user };
  }
};

const getUserFromLocalStorage = (): AuthUser | null => {
  if (typeof window === "undefined") return null;

  let user: string | object | null = localStorage.getItem("user");

  if (user) {
    return JSON.parse(user) as AuthUser;
  }

  return null;
};

export default function AppContextWrapper({ children }: Props) {
  const [state, dispatch] = useReducer(reducer, {
    user: getUserFromLocalStorage(),
  });

  return (
    <AppContext.Provider value={state}>
      <AppDispatchContext.Provider value={dispatch}>
        {children}
      </AppDispatchContext.Provider>
    </AppContext.Provider>
  );
}
