import { sql } from "drizzle-orm";
import { text, sqliteTable, integer } from "drizzle-orm/sqlite-core";

export const dns = sqliteTable("dns", {
    id: text("id").primaryKey(),
    domain: text("question").notNull(),
    type: text("type", 
                { enum: ["A", "AAAA", "CNAME", "TXT", "MX", "NS", "PTR", "SOA", "SRV"] }
            ).notNull(),
    ip: text("answer").notNull(),
    ttl: integer("ttl").notNull().default(sql`0`),
    createdAt: integer("createdAt", { mode: "timestamp" }).notNull().default(sql`CURRENT_TIMESTAMP`),
});