import { ResponseBuilder } from "@fermyon/spin-sdk";

//@ts-ignore
import { classify } from "component:image-classification-lib/image-classification"

export async function handler(req: Request, res: ResponseBuilder) {
    let img = await req.arrayBuffer();
    console.log(`[TS API]: Received image of size ${img.byteLength}. Running classification.`);
    try {
        let result = await classify(img);
        console.log(`[TS API]: Classification successful. Sending result ${result}`);
        res.send(result);
    } catch (e: any) {
        res.status(500);
        console.log(`[TS API]: Error running classification: ${e.message}, ${e.payload}.`);
        res.send("Internal Error");
    }
}

