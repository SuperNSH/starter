import { Text, View } from "nsh";
import { createForm, minLength, required } from "nsh-form";
import { createMutation } from "nsh-query";
import { API, ScreenTaken, registerScreen } from "./api";
import { Button, TextField } from "./ui";

/** Registers a screen by name; a success invalidates the dashboard, so its next load includes it. */
export function RegisterForm() {
  const register = createMutation<string, { id: string }>({
    invalidates: ["dashboard"],
    mutate: (name, { signal }) => registerScreen(name, signal),
  });
  const form = createForm({
    initial: { name: "" },
    onSubmit: async (values) => {
      try {
        await register.mutateAsync(values.name);
      } catch (error) {
        if (error instanceof ScreenTaken) return { fields: { name: "That name is already registered" } };
        return { form: "Couldn't reach the dashboard. Try again." };
      }
    },
  });
  const name = form.field("name", {
    rules: [required("Name the screen"), minLength(3, "Use at least 3 characters")],
  });
  return (
    <View class="gap-3">
      <TextField label="Screen name" placeholder="e.g. Lobby board" field={name} />
      <Button
        label="Register"
        disabled={form.isSubmitting()}
        onPress={() => void form.submit()}
      />
      <If when={form.error()}>
        {(message) => <Text class="text-sm text-destructive">{message()}</Text>}
      </If>
      <If when={register.data()}>
        {(data) => <Text class="text-sm text-muted-foreground">{`Registered as ${data().id}`}</Text>}
      </If>
    </View>
  );
}
