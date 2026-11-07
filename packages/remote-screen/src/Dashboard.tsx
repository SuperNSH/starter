import { Text, View } from "nsh";
import { createQuery, noRetries } from "nsh-query";
import { API, getJson, type Metric } from "./api";
import { Button, Card, Row } from "./ui";

/** The remote dashboard: cached 30 s, no automatic retries (the Retry button refetches). */
export function Dashboard() {
  const metrics = createQuery<Metric[]>({
    key: "dashboard",
    cache: { ttlMs: 30_000 },
    retry: noRetries,
    fetch: ({ signal }) => getJson<Metric[]>(`${API}/metrics`, signal),
  });
  return (
    <Card>
      <If when={metrics.isPending()}>
        <Text class="text-sm text-muted-foreground">Loading metrics…</Text>
      </If>
      <If when={metrics.isFailure()}>
        {(error) => (
          <View class="gap-2">
            <Text class="text-sm text-destructive">{`Couldn't load the dashboard: ${String(error())}`}</Text>
            <Button label="Retry" variant="outline" onPress={() => metrics.refetch()} />
          </View>
        )}
      </If>
      <If when={metrics.isSuccess()}>
        <Each of={metrics.data() ?? []} key={(metric) => metric.id}>
          {(metric) => <Row label={metric().label} value={metric().value} />}
        </Each>
      </If>
      <Button
        label="Refresh"
        variant="outline"
        disabled={metrics.isFetching()}
        onPress={() => metrics.refetch()}
      />
    </Card>
  );
}
