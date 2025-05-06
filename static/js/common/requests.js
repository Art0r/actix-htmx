window.get = async function get(url, loadingIndicator, entity) {
    try {
        const res = await fetch(url, { method: "GET"});

        if (res.ok) {
            entity.value = await res.json();
            loadingIndicator.value = false;
            return null;
        }

        loadingIndicator.value = false;
        return res.statusText;
    } catch (e) {
        loadingIndicator.value = false;
        return `Erro inesperado aconteceu: ${e}`
    }
}