import {SpeechClient, protos} from '@google-cloud/speech';
protos.google.cloud.speech.v1.LongRunningRecognizeRequest;

const client = new SpeechClient();

const run = async () => {
  console.log('Running...');
  await client.longRunningRecognize({
  });
  console.log('All done!');
};

run();