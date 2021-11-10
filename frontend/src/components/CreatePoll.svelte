<script lang="ts">
    import {createEventDispatcher} from "svelte";
    import {token as tokenStore} from "../stores/token";

    let dialog;
    let title;
    let optionsContainer;
    let token: string | null = null

    tokenStore.subscribe(it => {
        token = it
    })

    let options = [1, 2]

    const onFap = () => {
        dialog.show()
    }

    const addOption = () => {
        options = [...options, options.length + 1]
    }

    const dispatch = createEventDispatcher();

    const save = async () => {
        const values = Array.from(optionsContainer.children).filter(it => it.tagName.toLowerCase() === 'mwc-textfield').map(it => it.value)
        const payload = {
            title: title.value,
            options: values
        }
        const resp = await fetch("/api/polls", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify(payload)
        })

        const createdPoll = await resp.json()
        console.log({createdPoll})
        dispatch('pollCreate', createdPoll)
        dialog.close()
    }

    const onFapFinish = () => {
        options = [1, 2]
    }
</script>

<mwc-dialog bind:this={dialog} heading="Create new poll" on:closed={onFapFinish}>
    <div class="dialog-content">
        {#if token !== null}
            <mwc-textfield outlined bind:this={title} label="Title"/>
            <h3>Options</h3>
            <section class="options-container" bind:this={optionsContainer}>
                {#each options as optionNumber}
                    <mwc-textfield outlined label={`Option ${optionNumber}`}/>
                {/each}
                <mwc-button on:click={addOption}>
                    <mwc-icon>add</mwc-icon>
                    Add
                </mwc-button>
            </section>
        {:else}
            You must be logged in to create a poll
        {/if}
    </div>

    <mwc-button slot="primaryAction" on:click={save}>Save</mwc-button>

</mwc-dialog>

<mwc-fab class="new-fab" icon="add" on:click={onFap}></mwc-fab>

<style>
    .new-fab {
        position: absolute;
        bottom: 24px;
        right: 24px;
    }

    .dialog-content {
        padding: 1rem;
    }

    .dialog-content > h3 {
        padding: 1.5rem 0;
    }

    .options-container {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .options-container mwc-button {
        width: fit-content;
        align-self: center;
    }
</style>
