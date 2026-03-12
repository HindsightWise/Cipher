import { NextResponse } from "next/server";
import * as fs from "fs/promises";
import * as path from "path";

// Resolving physical paths to Cipher Engine's Cortexes residing in the /tmp/ environment from Phase 22
const MOTOR_CORTEX_PATH = "/tmp/Cipher_test50/cipher_engine/motor_cortex";
const SENSORY_CORTEX_PATH = "/tmp/Cipher_test50/cipher_engine/sensory_cortex";

export async function GET() {
  try {
    let responseText = null;
    let questionText = null;
    let monologueText = null;

    // 1. Read the Primary Output (cipher_response.txt)
    const responsePath = path.join(MOTOR_CORTEX_PATH, "cipher_response.txt");
    try {
      responseText = await fs.readFile(responsePath, "utf-8");
      // Consume the data completely physically (simulating neural firing)
      await fs.unlink(responsePath);
    } catch (e) {
      // File doesn't exist yet, normal polling state
    }

    // 2. Read the Question Output (question.txt)
    const questionPath = path.join(MOTOR_CORTEX_PATH, "question.txt");
    try {
      questionText = await fs.readFile(questionPath, "utf-8");
      await fs.unlink(questionPath); // Consume
    } catch (e) {
      // Not asking a question
    }

    // 3. Read the Core Structural Monologue (monologue.log)
    // We strictly DO NOT consume the monologue; it is a permanent structural graph representation.
    const monologuePath = path.join(SENSORY_CORTEX_PATH, "monologue.log");
    try {
      // Read the last 2000 characters of the monologue so we don't blow up the UI
      const stat = await fs.stat(monologuePath);
      let fileHandle = await fs.open(monologuePath, 'r');
      const bufferSize = Math.min(stat.size, 5000); // 5KB max
      const buffer = Buffer.alloc(bufferSize);
      const startPosition = Math.max(0, stat.size - bufferSize);
      
      await fileHandle.read(buffer, 0, bufferSize, startPosition);
      await fileHandle.close();
      
      monologueText = buffer.toString('utf-8');
    } catch (e) {
      // Monologue uninitialized or missing
    }

    return NextResponse.json({
      response: responseText,
      question: questionText,
      monologue: monologueText
    });
  } catch (error) {
    console.error("Polling System Error:", error);
    return NextResponse.json({ error: "Failed to poll motor cortex" }, { status: 500 });
  }
}
