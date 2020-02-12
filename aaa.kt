​
//
//  AppSettingViewController.swift
//  DecurretCFDApp
//
//  Created by okada on 2019/06/17.
//  Copyright © 2019 DeCurret Inc. All rights reserved.
//
final class AppSettingViewController: CFDBaseViewController, FooterTabSubViewController {
    
    fun setup() {}
    @IBOutlet lateinit var tableView: UITableView
    @IBOutlet lateinit var appVersionLabel: SmartLabel
    private var items: List<String> = listOf()
    private val settingItems: List<SelectableItem> = listOf(SelectableItem.authentication, SelectableItem.initialDisplay, SelectableItem.logout)
    enum class SelectableItem (val rawValue: String) {
        authentication("二段階認証設定（照会のみ）"), initialDisplay("アプリ起動時 表示画面設定"), logout("ログアウト");
    
        companion object {
            operator fun invoke(rawValue: String) = SelectableItem.values().firstOrNull { it.rawValue == rawValue }
        }
    }
    
    override fun onEnterForeground() {
        super.onEnterForeground()
        AppSetting.shared.lastDisplayScreen = .appConfig
        val footerTabViewController = ScreenService.shared.foregroundController as? FooterTabViewController
        if (footerTabViewController != null) {
            footerTabViewController.setScreenTitle("アプリ設定")
            footerTabViewController.isHiddenHeader(false)
        }
        setItems()
        tableView.reloadData()
    }
    
    override fun loadView() {
        val view = UINib(nibName = "AppSettingView", bundle = null).instantiate(withOwner = this, options = null).firstOrNull() as? UIView
        if (view != null) {
            this.view = view
        }
    }
    
    override fun viewDidLoad() {
        super.viewDidLoad()
        setInformationLabel()
        setItems()
        settingTableView()
    }
    @IBOutlet lateinit var informationLabel: SmartLabel
    
    private fun setInformationLabel() {
        informationLabel.text = "メールアドレス（ログインID）およびパスワードは、PCサービスサイトまたはDeCurretアプリ（現物取引・電子マネーチャージ）よりご変更ください"
    }
    
    private fun settingTableView() {
        tableView.delegate = this
        tableView.dataSource = this
        tableView.register(UINib(nibName = "AppSettingCell", bundle = null), forCellReuseIdentifier = "AppSettingCell")
    }
    
    // detail項目は仮置き
    private fun setItems() {
        val firstScreen = AppSetting.shared.firstDisplayScreen.titleTab
        items = listOf("${SelectableItem.authentication.rawValue},", "${SelectableItem.initialDisplay.rawValue},${firstScreen}", "${SelectableItem.logout.rawValue},")
    }
    
    /// 遷移先画面の指定
    private///
    /// - Parameter itemsName: 遷移先の項目名
     fun transitionToNextView(itemName: SelectableItem) {
        // 各遷移先のメソッドを作成
        val items: Map<SelectableItem, () -> Unit> = mapOf(SelectableItem.authentication to moveToAuthenticationSettingViewController, SelectableItem.initialDisplay to moveToInitialScreenSettingViewController, SelectableItem.logout to logout)
        val selectedItem = items[itemName] ?: return
        selectedItem()
    }
    
    private fun logout() {
        LoginService.shared.logout {
            val tabView = ScreenService.shared.rootViewController as? FooterTabViewController
            if (tabView == null) {
                return@logout
            }
            tabView?.selectHome()
        }
    }
    
    private fun moveToLoginSettingViewController() {
        val tabView = ScreenService.shared.rootViewController as? FooterTabViewController
        if (tabView == null) {
            return
        }
        tabView?.setTabScreen(type = LoginSettingViewController.self)
    }
    
    private fun moveToAuthenticationSettingViewController() {
        val tabView = ScreenService.shared.rootViewController as? FooterTabViewController
        if (tabView == null) {
            return
        }
        tabView?.setTabScreen(type = AuthenticationSettingViewController.self)
    }
    
    private fun moveToInitialScreenSettingViewController() {
        val tabView = ScreenService.shared.rootViewController as? FooterTabViewController
        if (tabView == null) {
            return
        }
        tabView?.setTabScreen(type = InitialScreenSettingViewController.self)
    }
}
//FIXME: @SwiftKotlin - Kotlin does not support inheritance clauses in extensions:  : UITableViewDelegate
​
fun AppSettingViewController.tableView(tableView: UITableView, indexPath: IndexPath) {
    val selectedItem = settingItems[indexPath.item]
    transitionToNextView(selectedItem)
    tableView.deselectRow(at = indexPath, animated = true)
}
//FIXME: @SwiftKotlin - Kotlin does not support inheritance clauses in extensions:  : UITableViewDataSource
​
fun AppSettingViewController.tableView(tableView: UITableView, section: Int) : Int =
    this.items.size
​
fun AppSettingViewController.tableView(tableView: UITableView, indexPath: IndexPath) : UITableViewCell {
    val cell = tableView.dequeueReusableCell(withIdentifier = "AppSettingCell", for = indexPath) as? AppSettingCell ?: return UITableViewCell()
    val items = this.items[indexPath.row].components(separatedBy = ",")
    cell.setSettingItem(title = items[0], detail = items[1])
    if (indexPath.row == 2) {
        cell.accessoryType = .none
        cell.titlelabel?.textColor = Colors.red
    }
    return cell
}