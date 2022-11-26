"use client";

import React from "react";
import useAuth from "@hooks/use-auth";

export default function RoomJoinPage() {
  const { user } = useAuth();

  return (
    <h1>
      HELLO {user?.name} ID: {user?.id}
    </h1>
  );
}
