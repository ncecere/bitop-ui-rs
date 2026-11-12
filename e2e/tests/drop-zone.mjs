export default async ({ page, check, axe, button }) => {
  const zone = page.getByRole("group", { name: "Drag and drop documents here, or" });
  const input = zone.locator("input[type=file]");
  const rejected = zone.locator("[data-drop-zone-rejected]");
  const listed = () => zone.locator("[data-drop-zone-files] li").allTextContents();
  const formFiles = () => page.evaluate(() => new FormData(document.getElementById("upload-form")).getAll("documents").map((f) => f.name).filter(Boolean));
  const file = (name, mimeType, bytes = 10) => ({ name, mimeType, buffer: Buffer.alloc(bytes, 1) });

  // A labelled group with a real button and a real file input.
  const choose = zone.getByRole("button", { name: "Choose files" });
  check("described", await choose.evaluate((b) => document.getElementById(b.getAttribute("aria-describedby")).textContent.trim()), "PDF or images, up to 1 MB each. Two files at most.");
  check("input", await input.evaluate((i) => [i.name, i.multiple, i.accept, i.hidden, i.form.id, i.form.enctype]), ["documents", true, ".pdf,image/*", true, "upload-form", "multipart/form-data"]);
  check("status", [await rejected.getAttribute("role"), await rejected.textContent()], ["status", ""]);

  // The button opens the file picker, from the keyboard too.
  await choose.focus();
  const [chooser] = await Promise.all([page.waitForEvent("filechooser"), page.keyboard.press("Enter")]);
  check("buttonOpensPicker", chooser.isMultiple());
  await chooser.setFiles([file("report.pdf", "application/pdf", 2048), file("photo.png", "image/png", 300)]);
  check("accepts", [await listed(), await formFiles(), await rejected.textContent()], [["report.pdf2 KB", "photo.png300 B"], ["report.pdf", "photo.png"], ""]);

  // Files that fail accept or the size limit are named, and left out of the form.
  await input.setInputFiles([file("setup.exe", "application/x-msdownload"), file("notes.PDF", ""), file("huge.png", "image/png", 1024 * 1024 + 1)]);
  check("rejects", [await listed(), await formFiles()], [["notes.PDF10 B"], ["notes.PDF"]]);
  check("says", await rejected.textContent(), "setup.exe isn't an accepted file type. huge.png is larger than 1 MB.");
  check("axeRejected", await axe(), []);
  // No more than two files.
  await input.setInputFiles([file("a.pdf", "application/pdf"), file("b.pdf", "application/pdf"), file("c.pdf", "application/pdf")]);
  check("countLimit", [await formFiles(), await rejected.textContent()], [["a.pdf", "b.pdf"], "c.pdf was left out: no more than 2 files."]);
  // A good choice clears the message.
  await input.setInputFiles([file("ok.pdf", "application/pdf")]);
  check("clearsMessage", [await formFiles(), await rejected.textContent()], [["ok.pdf"], ""]);

  // Dropping: the zone shows it is a target, and the files go through the same checks.
  const dropData = () =>
    page.evaluateHandle(() => {
      const data = new DataTransfer();
      data.items.add(new File(["x".repeat(20)], "dropped.png", { type: "image/png" }));
      data.items.add(new File(["x"], "virus.bat", { type: "application/x-bat" }));
      return data;
    });
  await page.evaluate(() => {
    window.dropEvents = [];
    document.addEventListener("bitop:files", (event) =>
      window.dropEvents.push([event.detail.files.map((f) => f.name), event.detail.rejections.map((r) => `${r.file.name}:${r.reason}`)]),
    );
  });
  let data = await dropData();
  await zone.dispatchEvent("dragenter", { dataTransfer: data });
  check("dragging", await zone.evaluate((z) => z.hasAttribute("data-dragging")));
  const over = await zone.evaluate((z) => {
    const event = new DragEvent("dragover", { bubbles: true, cancelable: true });
    z.dispatchEvent(event);
    return event.defaultPrevented;
  });
  check("allowsDrop", over);
  await zone.dispatchEvent("dragleave", { dataTransfer: data });
  check("dragLeave", await zone.evaluate((z) => z.hasAttribute("data-dragging")), false);
  await zone.dispatchEvent("dragenter", { dataTransfer: data });
  data = await dropData();
  await zone.dispatchEvent("drop", { dataTransfer: data });
  check("drop", [await zone.evaluate((z) => z.hasAttribute("data-dragging")), await listed(), await formFiles(), await rejected.textContent()], [false, ["dropped.png20 B"], ["dropped.png"], "virus.bat isn't an accepted file type."]);
  check("event", await page.evaluate(() => window.dropEvents), [[["dropped.png"], ["virus.bat:type"]]]);

  // One file at a time: a second one is left out.
  const single = page.getByRole("group", { name: "Drag and drop a picture here, or" });
  check("single", await single.locator("input[type=file]").evaluate((i) => [i.multiple, i.accept]), [false, "image/png,image/jpeg"]);
  check("noDescription", await single.getByRole("button", { name: "Choose a picture" }).getAttribute("aria-describedby"), null);
  data = await page.evaluateHandle(() => {
    const d = new DataTransfer();
    d.items.add(new File(["x"], "one.png", { type: "image/png" }));
    d.items.add(new File(["x"], "two.jpg", { type: "image/jpeg" }));
    d.items.add(new File(["x"], "three.gif", { type: "image/gif" }));
    return d;
  });
  await single.dispatchEvent("drop", { dataTransfer: data });
  check("singleKeepsOne", [await single.locator("[data-drop-zone-files] li").allTextContents(), await single.locator("[data-drop-zone-rejected]").textContent()], [["one.png1 B"], "two.jpg was left out: one file only. three.gif isn't an accepted file type."]);

  // Busy and disabled take nothing.
  const busy = page.locator("[data-drop-zone]", { hasText: "Uploading…" });
  check("busy", [await busy.locator("button").isDisabled(), await busy.locator("button").getAttribute("aria-busy"), await busy.locator("input").isDisabled()], [true, "true", true]);
  const locked = page.locator("[data-drop-zone][data-disabled]");
  data = await dropData();
  await locked.dispatchEvent("dragenter", { dataTransfer: data });
  await locked.dispatchEvent("drop", { dataTransfer: data });
  check("disabled", [await locked.locator("button").isDisabled(), await locked.evaluate((z) => z.hasAttribute("data-dragging")), await locked.locator("[data-drop-zone-files] li").count()], [true, false, 0]);
};
