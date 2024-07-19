import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable


WebUI.comment('TEST CASE: Import agencies CSV to the created supplier')


'GET INITIAL SYSTEM EVENTS COUNT TO VERIFY THE OPERATUION LATER'//to verify the examined operation is the expected one
WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/PushNotificationMarkAsRead'))
getRecentNotifications = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/PushNotificationSearch'))
eventsInitialCount = WS.getElementPropertyValue(getRecentNotifications,'totalCount')
eventsActualCount = eventsInitialCount


'SEND ERQUEST TO UPLOAD A FILE'
fileName = 'agenciesImport.csv'
folderUrl = 'tmp'
uploadFile = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Assets/AssetFileUploadLocalByFileName',[
	('fileName') : fileName,
	('folderUrl') : folderUrl
	]))
WS.verifyElementPropertyValue(uploadFile,'[0].name', fileName)


'VERIFY THE FILE HAS BEEN UPLOADED AND GET FILE URL'
uploadedFileGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Assets/AssetGetList',[
	('folderName') : folderUrl,
	('keyword') : fileName
	]))
fileUrl = WS.getElementPropertyValue(uploadedFileGet,'results[0].relativeUrl')


'RUN FILE IMPORT'
fileUrl = fileUrl.substring(1) //remove the initial slash symbol so the url would fit the format of the request
selectedDelimeter = ","
propertiesimportRun = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importRunAgencies',[
	('supplierId') : GlobalVariable.supplierId,
	('fileUrl') : fileUrl,
	('delimeter') : selectedDelimeter
	]))
jobId = WS.getElementPropertyValue(propertiesimportRun,'jobId')


'VERIFY THE OPERATION IS LOGGED IN THE SYSTEM EVENTS'
while (eventsActualCount == eventsInitialCount) {
	getRecentNotifications = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/PushNotificationSearch'))
	eventsActualCount = WS.getElementPropertyValue(getRecentNotifications,'totalCount')
}


'VERIFY IMPORT OPERATION HAS BEEN FINSHED SUCCESSFULLY'
def importFinishedStatus
while (importFinishedStatus == null) {
	getRecentNotifications = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/PushNotificationSearch'))
	//assert  "title":"Indexation process",
	importFinishedStatus = WS.getElementPropertyValue(getRecentNotifications,'notifyEvents[0].finished')
}
verifyResult = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/PushNotificationSearch'))
WS.verifyElementPropertyValue(verifyResult,'notifyEvents[0].errors', [])


