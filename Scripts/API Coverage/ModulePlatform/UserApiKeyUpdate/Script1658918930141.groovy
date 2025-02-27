import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper as JsonSlurper


//STEP | set new API key to user not admin 
WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/ApiKeyUpdate', [
	('userName') : GlobalVariable.userName,
	('api_key') : GlobalVariable.userApiKey,
	('userId') : GlobalVariable.userId,
	('apiKeyStatus') : GlobalVariable.apiKeyStatus,
	('apiKeyId') : GlobalVariable.apiKeyId
	]))

responseApiKey = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/ApiKeyGet', [
	('userId') : GlobalVariable.userId
	]))
WS.verifyElementPropertyValue(responseApiKey, '[0].apiKey', GlobalVariable.userApiKey)
WS.verifyElementPropertyValue(responseApiKey, '[0].isActive', GlobalVariable.apiKeyStatus)
GlobalVariable.apiKeyId = (WS.getElementPropertyValue(responseApiKey, '[0].id'))
WebUI.comment('API KEY ID : ' + GlobalVariable.apiKeyId)