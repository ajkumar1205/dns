import { drizzle } from 'drizzle-orm/libsql';
import { createClient } from '@libsql/client';
import * as mainSchema from "../drizzle/schema";
import * as dnsSchema from "../drizzle/dns-schema";

const client = createClient({
    url: Bun.env.DATABASE_URL!,
});

const dnsClient = createClient({
    url: Bun.env.DNS_DATABASE_URL!,
});

export const db = drizzle(client, { schema: mainSchema });
export const dns = drizzle(dnsClient, { schema: dnsSchema });
