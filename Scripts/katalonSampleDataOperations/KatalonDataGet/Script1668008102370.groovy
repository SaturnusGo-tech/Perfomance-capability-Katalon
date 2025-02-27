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


WebUI.comment('TEST CASE: collect katalon related test data')


'FIRST BUILD INDEX'
indexBuild = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'),
	null)


'GET KATALON USERID'
'SEARCH FOR THE KATALON USER ID'
userSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/UserSearch', [
	('searchPhrase') : GlobalVariable.katalonUserName,
	]))
GlobalVariable.katalonUserId = WS.getElementPropertyValue(userSearch, 'users[0].id')


'GET KATALON KATALON CATALOGID'
searchKatalonCatalog = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Catalog/CatalogsSearch', [
	('keyword') : GlobalVariable.catalogName
	]))
GlobalVariable.catalogId = WS.getElementPropertyValue(searchKatalonCatalog , 'results[0].id')


'GET KATALON CATEGORYID'
catalogCategoriesSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Catalog/CategoriesSearch',[
	('catalogId') : GlobalVariable.catalogId
	]))
GlobalVariable.categoryId = WS.getElementPropertyValue(catalogCategoriesSearch,'items[0].id')


println 'katalonUserId ' + GlobalVariable.katalonUserId
println 'catalogId ' + GlobalVariable.catalogId
println 'categoryId ' + GlobalVariable.categoryId 
