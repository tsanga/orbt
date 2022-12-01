import { useEffect, useContext, useRef } from "react";
import { AppContext, AppDispatchContext } from "@/context";
import { useMutation, graphql } from "react-relay";
import { AuthUser } from "@/context";

export type Auth = {
  user: AuthUser | null;
  isLoggedIn: boolean;
};

export default function useAuth(): Auth {
  const state = useContext(AppContext);
  const user = state.user;

  return { user, isLoggedIn: !!user };
}
