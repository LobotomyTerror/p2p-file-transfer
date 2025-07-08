function fileSubmit() {
    const fileSubmission = document.getElementById("fileupload");

    fileSubmission.addEventListener("submit", (ev) => {
        ev.preventDefault();
        let targetFiles = ev.target[0];

        const data = new FormData();

        for (file of targetFiles.files)
            data.append('files', file);

        console.log(data);

        fetch(
            'http://127.0.0.1:8000/file-upload',
            {
                method: "POST",
                body: data
            }
        ).then(
            response => {
                if (!response.ok) throw new Error("Failed to upload files");
                // return response.json;
                console.log(response);
            }
        )
    })
}


window.addEventListener('load', fileSubmit);
