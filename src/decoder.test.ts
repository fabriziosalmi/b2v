import { Decoder } from './decoder';
import { expect } from 'chai';
import { Result } from 'anyhow';

// Mocking external dependencies like ffmpeg execution is crucial for unit testing.

describe('Decoder', () => {
    const INPUT_PATH = './input.mp4';
    const OUTPUT_PATH = './output.mp4';

    describe('Decoder Initialization', () => {
        it('should initialize correctly', () => {
            const decoder = new Decoder(INPUT_PATH, OUTPUT_PATH);
            expect(decoder.input_path).to.equal(INPUT_PATH);
            expect(decoder.output_path).to.equal(OUTPUT_PATH);
        });
    });

    describe('Decoder Run', () => {
        it('should successfully run the decoding process', async () => {
            // Testing the flow where ffmpeg reads input and buffers frames.
            const decoder = new Decoder(INPUT_PATH, OUTPUT_PATH);

            // In a real scenario, this would require mocking ffmpeg output to simulate frame data.
            await expect(decoder.run()).to.be.fulfilled;
        });

        it('should handle failure when spawning ffmpeg', async () => {
            // Test case for failure in spawning the decoder command.
            const decoder = new Decoder(INPUT_PATH, OUTPUT_PATH);

            // Requires mocking Command::spawn to throw an error.
            await expect(decoder.run()).to.be.rejectedWith('Failed to spawn ffmpeg decodder');
        });

        it('should handle failure when reading header frame', async () => {
            // Test case for failure when reading the initial frame buffer.
            const decoder = new Decoder(INPUT_PATH, OUTPUT_PATH);

            // Requires mocking stdout.read_exact to fail.
            await expect(decoder.run()).to.be.rejectedWith('Failed to read header frame');
        });
    });
});