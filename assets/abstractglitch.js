// Memorizza il contenuto originale del div principale come stringa
const originalContent = `
<div id="text-container-en">
    <p>
        <span class="text-red-500 weird">Anomaly</span>. <span class="text-red-500 weird">Irregularity</span>, <span class="text-red-500 weird">deviation</span> from the general rule, or from a structure, from a type considered normal.
        Man is guided by a variety of factors, such as emotions, beliefs, religion, and social context. Factors like these can create barriers and hinder the ability to effectively learn from one’s mistakes. In addition, changes require an often unfathomable sacrifice.
    </p>
    <p>
        Misalignments, <span class="text-red-500 weird">deviations from the line</span>, are often sources of contamination and enrichment. <span class="text-red-500 weird">Anomalies</span> can challenge our preconceived ideas or traditional beliefs about reality. When we encounter <span class="text-red-500 weird">something unexpected</span> or that does not fit our mental patterns, we are forced to reconsider our beliefs and seek <span class="text-red-500 weird">alternative answers</span>. This process, through the acceptance of new ideas and creative flows, challenges the universally known and shakes the status quo. <span class="text-red-500 weird">Anomalies</span> can instill courage to face reality.
    </p>

    <br>
</div>
`;

function randomizeText() {
    // Ottieni il div principale contenente il testo originale
    const textContainer = document.getElementById('text-container-en');

    // Ottieni tutti gli span con la classe "weird"
    const spans = textContainer.querySelectorAll('span.weird');

    // Seleziona un tag span con la classe "weird" in modo casuale
    const randomSpan = spans[Math.floor(Math.random() * spans.length)];

    // Memorizza il testo originale dello span selezionato
    const originalText = randomSpan.textContent;

    // Genera un testo alterato con caratteri casuali
    let alteredText = originalText.split('').map(char => {
        // Sostituisci casualmente alcuni caratteri con caratteri diversi
        return Math.random() > 0.8 ? char : String.fromCharCode(Math.floor(Math.random() * (126 - 33 + 1)) + 33);
    }).join('');

    // Modifica il contenuto dello span selezionato con il testo alterato
    randomSpan.textContent = alteredText;

    // Imposta un timeout per ripristinare il contenuto originale dopo 200 millisecondi
    setTimeout(() => {
        // Ripristina il contenuto originale del div principale utilizzando la variabile
        textContainer.innerHTML = originalContent;
    }, 500);

    // Imposta un timeout per eseguire nuovamente randomizeText dopo un periodo random tra 10 e 20 secondi
    const randomDelay = Math.floor(Math.random() * 11) + 10; // Genera un intervallo tra 10 e 20 secondi
    setTimeout(randomizeText, randomDelay * 1000);
}

// Avvia il processo al caricamento della pagina
document.addEventListener('DOMContentLoaded', randomizeText);

