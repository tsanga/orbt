"use client";

import React from "react";
import type { pageQuery as PageQuery } from "@gql/pageQuery.graphql";
import { graphql, useLazyLoadQuery } from "react-relay";

const PageQuery = graphql`
  query pageQuery($id: Int!) {
    user(id: $id) {
      name
    }
  }
`;

export default function RoomJoinPage() {
  const data = useLazyLoadQuery<PageQuery>(PageQuery, { id: 0 });
  return <>{data.user.get?.name}</>;
}
