import * as styles from "./room-top-bar.css";
import Logo from "@assets/svg/logo-long.svg";
import { graphql, useFragment } from "react-relay";
import { roomTopBarTitle$key } from "@gql/roomTopBarTitle.graphql";
import RoomTopBarTitle from "./room-top-bar-title";

type Props = {
  room: roomTopBarTitle$key;
};

export default function TopBar({ room }: Props) {
  return (
    <nav className={styles.nav}>
      <Logo />
      <RoomTopBarTitle room={room} />
    </nav>
  );
}
