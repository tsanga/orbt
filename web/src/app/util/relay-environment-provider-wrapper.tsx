"use client";

import { RelayEnvironmentProvider } from "react-relay";
import Environment from "@/relay-env";

interface Props {
  children: React.ReactNode;
}

export default function RelayEnvironmentProviderWrapper({ children }: Props) {
  return (
    <RelayEnvironmentProvider environment={Environment}>
      {children}
    </RelayEnvironmentProvider>
  );
}
