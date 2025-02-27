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
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys


WebUI.comment('TEST CASE: OPUS catalog search')

//ListentriesSearch doesn't return requested product in results
//probably a backend workflow pecularity
//Need to discuss via Danil
//Until then I will turn this test case off


'SEARCH FOR THE OPUS PRODUCT IN LISTENTRIES'
productSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Catalog/ListentriesSearch', [
	('keyword') : GlobalVariable.OPUS_productSKU
	]))
WS.containsString(productSearch, GlobalVariable.OPUS_productName, false)


'GET PRODUCT BY ID'
productGet = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Catalog/ProductsGetByIdAndGroup', [
	('id') : GlobalVariable.OPUS_productId
	]))
WS.verifyElementPropertyValue(productGet, '[0].name', GlobalVariable.OPUS_productName)
