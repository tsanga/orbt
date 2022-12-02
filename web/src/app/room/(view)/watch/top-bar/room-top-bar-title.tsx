"use client";

import * as styles from "./room-top-bar-title.css";
import { graphql, useFragment } from "react-relay";
import { roomTopBarTitle$key } from "@gql/roomTopBarTitle.graphql";

type Props = {
  room: roomTopBarTitle$key;
};

export default function TopBar({ room }: Props) {
  let data = useFragment(
    graphql`
      fragment roomTopBarTitle on Room {
        name
      }
    `,
    room
  );

  return <h1 className={styles.title}>{data.name}</h1>;
}
