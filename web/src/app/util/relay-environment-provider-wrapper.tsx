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

export type Action = { type: "setEnvironment"; environment: Environment };

type Props = {
  children: React.ReactNode;
};

const reducer = (state: State, action: Action): State => {
  console.log("hi");
  switch (action.type) {
    case "setEnvironment":
      console.log("setEnv");
      return { ...state, environment: action.environment };
  }
};

const environment = createEnvironment({ configName: "root-environment" });

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
