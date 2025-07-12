function fileSubmit() {
    const fileSubmission = document.getElementById("fileupload");

    fileSubmission.addEventListener("submit", async (ev) => {
        ev.preventDefault();
        let targetFiles = ev.target[0];

        const data = new FormData();

        const iv = window.crypto.getRandomValues(new Uint8Array(12));
        const key = await window.crypto.subtle.generateKey(
            {
                name: "AES-GCM",
                length: 256,
            },
            true,
            ['encrypt', 'decrypt'],
        );

        for (file of targetFiles.files) {
            const arrayBuffer = await file.arrayBuffer();

            const encryptValue = await window.crypto.subtle.encrypt(
                {name: "AES-GCM", iv},
                key,
                arrayBuffer
            );
            // console.log(encryptValue);
            data.append("files", new Blob([encryptValue]), file.name);
        }

        const rawKey = await window.crypto.subtle.exportKey("raw", key);
        const key64 = btoa(String.fromCharCode(...new Uint8Array(rawKey)));

        const iv64 = btoa(String.fromCharCode(...iv));

        // data.append("key", key64);
        // data.append("iv", iv64);

        console.log(data);

        fetch(
            'http://127.0.0.1:8000/file-upload',
            {
                method: "POST",
                body: data
            }
        ).then(
            response => {
                if (!response.ok)
                    throw new Error("Failed to upload files");
                return response.json();
            }
        ).then(
            json => {
                console.log(json);
            }
        );
    });
}


window.addEventListener('load', fileSubmit);
