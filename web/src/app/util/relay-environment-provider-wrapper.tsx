"use client";

import { RelayEnvironmentProvider } from "react-relay";
import createEnvironment from "@/relay-env";
import { Environment } from "relay-runtime";
import { createContext, Dispatch, useReducer } from "react";

export const RelayContext = createContext<State | null>(null);
export const RelayDispatchContext = createContext<Dispatch<Action>>(() => {});

export type State = {
  environment: Environment;
};

export type Action = { type: "SET_ENVIRONMENT"; environment: Environment };

type Props = {
  children: React.ReactNode;
};

const reducer = (state: State, action: Action): State => {
  switch (action.type) {
    case "SET_ENVIRONMENT":
      console.log(
        `relay env changed from ${state.environment.configName} to ${action.environment.configName}`
      );
      return { ...state, environment: action.environment };
  }
};

const environment = createEnvironment({ configName: "root-anon-environment" });

export default function RelayEnvironmentProviderWrapper({ children }: Props) {
  const [state, dispatch] = useReducer(reducer, {
    environment: environment,
  });

  return (
    <RelayContext.Provider value={state}>
      <RelayDispatchContext.Provider value={dispatch}>
        <RelayEnvironmentProvider environment={state.environment}>
          {children}
        </RelayEnvironmentProvider>
      </RelayDispatchContext.Provider>
    </RelayContext.Provider>
  );
}
