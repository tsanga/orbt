"use client";

import { ApolloClient, ApolloProvider } from '@apollo/client';

interface Props<T> {
    children: React.ReactNode;
    client: ApolloClient<T>,
}

export default function ApolloProviderWrapper<T>({ children, client }: Props<T>) {
  return <ApolloProvider client={client}>{children}</ApolloProvider>;
}