## Guardrails IA Anti-dette Mobile

1. Chaque écran est défini par un `ScreenContract` (screenName, intent, layout, scrollAllowed).
2. Aucun hook `useParams`, `useNavigate`, `fetch`, `supabase` ou accès direct au DOM dans le Screen.
3. Les données circulent via `usecases` injectés, pas via la vue. Ajouter des TODO pour `usecase injection`, `data binding`, `navigation handling`.
4. La page Web est simplement un wrapper : `export default function Page() { return <YourScreen /> }`.
5. Documenter les nouvelles screens dans `docs/framework/Miyukini Framework - Screens`.
