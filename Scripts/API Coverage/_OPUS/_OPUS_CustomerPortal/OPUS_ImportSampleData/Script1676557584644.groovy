import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import internal.GlobalVariable


WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),null)


'IMPORT OPUS BACKEND SAMPLE DATA FROM A FILE'
GlobalVariable.fileName = 'OPUS_backendSamples.zip'
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePlatform/ImportSampleData'),null)