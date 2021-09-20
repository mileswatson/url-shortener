<script>
    import superagent from "superagent";

    let url = "";
    let request = null;

    function click() {
        request = superagent.post(`/api/shorten?url=${url}`);
    }

    function getUrl(key) {
        return `http://${window.location.host}/${key}`;
    }
</script>

<div class="box">
    {#if request == null}
        <div class="field has-addons">
            <div class="control">
                <input
                    class="input"
                    type="text"
                    bind:value={url}
                    placeholder="URL"
                />
            </div>
            <div class="control">
                <button class="button is-info" on:click={click}>Shorten</button>
            </div>
        </div>
    {:else}
        {#await request}
            <p>Loading...</p>
        {:then response}
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">Done!</p>
                </header>
                <div class="card-content">
                    <a
                        class="content"
                        href={getUrl(response.text)}
                        target="_blank">{getUrl(response.text)}</a
                    >
                </div>
                <footer class="card-footer">
                    <button
                        class="card-footer-item button"
                        on:click={() => (request = null)}>Back</button
                    >
                    <button
                        class="card-footer-item button is-info"
                        on:click={() =>
                            navigator.clipboard.writeText(
                                getUrl(response.text)
                            )}>Copy</button
                    >
                </footer>
            </div>
        {:catch}
            <p>Something went wrong!</p>
        {/await}
    {/if}
</div>
