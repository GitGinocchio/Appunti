


Al posto dell'iframe voglio aggiungere https://docusaurus.io/

Stack:
- Rust
- docosaurus
- Notion API (notion-rs)

Quello che deve fare il setup di build del sito di Notion e' questo:
1. Ottiene la build precedente (se presente) da github attraverso gli artifacts
2. Cercare in modo ricorsivo le pagine di Notion
3. Per ogni pagina controllare un file (se presente la build precedente) chiamato manifest.json in cui saranno presenti gli ultimi timestamp di aggiornamento per ogni pagina
4. Se l'ultima modifica e' piu' recente di quella indicata scarica quella specifica pagina
5. Applica Frontmatter sopra la pagina
5. Effettua dei refactor se necessario per adattarlo a docosaurus
7. Salva la pagina insieme al resto della build