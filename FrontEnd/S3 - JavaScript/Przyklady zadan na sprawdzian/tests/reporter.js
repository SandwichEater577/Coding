const statuses = {
  passed: '✅ (passed)',
  failed: '❌ (failed)',
};

class CustomReporter {
  onTestResult(test, x) {
    const path = x.testFilePath;
    const regex = /zadanie(\d+)/;
    const match = path.match(regex);
    const taskNumber = match[1];
    console.log();
    console.log(`Zadanie ${taskNumber}`);

    x.testResults.forEach(({ status, title }) => {
      console.log(`${statuses[status]}  ${title}`);
    });
  }
}

module.exports = CustomReporter;
