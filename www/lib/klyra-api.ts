import fetch from "node-fetch";

export async function getApiKey(username: string): Promise<string> {
  const res = await fetch(
    `${process.env.klyra_API_BASE_URL}/users/${username}`,
    {
      method: "POST",
      headers: {
        Authorization: `Bearer ${process.env.klyra_ADMIN_SECRET}`,
      },
    }
  );

  if (res.ok) {
    return res.text();
  } else {
    throw new Error("could not get api key.");
  }
}
