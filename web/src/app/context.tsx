"use client";

import {
  createContext,
  Dispatch,
  useReducer,
  useEffect,
  useContext,
  useRef,
} from "react";
import { User } from "@domain/models";
import { stat } from "fs";
import {
  RelayContext,
  RelayDispatchContext,
} from "./util/relay-environment-provider-wrapper";
import createEnvironment from "@/relay-env";
import useAuth from "@hooks/use-auth";
import { graphql, useMutation } from "react-relay";
import type { contextCreateUserMutation } from "@gql/contextCreateUserMutation.graphql";

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
  let toReturn;

  switch (action.type) {
    case "SET_USER":
      toReturn = { ...state, user: action.user };
      break;
  }

  if (typeof window !== "undefined") {
    localStorage.setItem("user", JSON.stringify(toReturn));
  }

  return toReturn;
};

const initializer = (): State => {
  if (typeof window === "undefined") return { user: null };

  let user = localStorage.getItem("user");

  if (user) {
    return JSON.parse(user);
  }

  return { user: null };
};

export default function AppContextWrapper({ children }: Props) {
  const [state, dispatch] = useReducer(reducer, {}, initializer);
  const ref = useRef(false);

  const relayDispatchContext = useContext(RelayDispatchContext);
  const relayContext = useContext(RelayContext);

  const [commitMutation, _] = useMutation<contextCreateUserMutation>(
    graphql`
      mutation contextCreateUserMutation($name: String) {
        createUser(name: $name) {
          id
          name
          token {
            token
          }
        }
      }
    `
  );

  const user = state.user;

  useEffect(() => {
    if (!user) {
      if (!ref.current) {
        commitMutation({
          variables: {},
          onCompleted: (response) => {
            if (response.createUser) {
              dispatch({
                type: "SET_USER",
                user: response.createUser as AuthUser,
              });
            }
          },
          onError: (err) => {
            console.log(err);
          },
        });

        ref.current = true;
      }
    }
  }, [user, ref]);

  useEffect(() => {
    if (
      user?.token?.token &&
      relayContext?.environment &&
      relayContext.environment.configName === "root-anon-environment"
    ) {
      relayDispatchContext({
        type: "SET_ENVIRONMENT",
        environment: createEnvironment(
          {
            configName: "root-auth-environment",
          },
          { Authorization: user.token.token }
        ),
      });
    }
  }, [user, relayContext]);

  return (
    <AppContext.Provider value={state}>
      <AppDispatchContext.Provider value={dispatch}>
        {children}
      </AppDispatchContext.Provider>
    </AppContext.Provider>
  );
}
