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

WebUI.waitForElementPresent(findTestObject('Parts/Add Record/button_Add Record'), 10)

WebUI.setText(findTestObject('Parts/Add Record/input_Part Number Filter'), number)

WebUI.delay(4)

WebUI.verifyElementText(findTestObject('Parts/Update/div_Part Number'), number, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Parts/Update/div_Part Name'), name, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Parts/Update/div_Part Type'), 'SCIENCE/LABORATORY EQUIPMENT', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Parts/Update/div_Part Area'), 'Technology Department', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Parts/Update/div_Cost'), cost, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Parts/Update/div_Min Stock'), stock, FailureHandling.STOP_ON_FAILURE)