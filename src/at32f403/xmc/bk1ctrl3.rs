#[doc = "Register `BK1CTRL3` reader"]
pub type R = crate::R<BK1CTRL3_SPEC>;
#[doc = "Register `BK1CTRL3` writer"]
pub type W = crate::W<BK1CTRL3_SPEC>;
#[doc = "Field `EN` reader - Memory bank enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Memory bank enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADMUXEN` reader - Address and data multiplexing enable"]
pub type ADMUXEN_R = crate::BitReader;
#[doc = "Field `ADMUXEN` writer - Address and data multiplexing enable"]
pub type ADMUXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEV` reader - Memory device type"]
pub type DEV_R = crate::FieldReader;
#[doc = "Field `DEV` writer - Memory device type"]
pub type DEV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXTMDBW` reader - External memory data bus width"]
pub type EXTMDBW_R = crate::FieldReader;
#[doc = "Field `EXTMDBW` writer - External memory data bus width"]
pub type EXTMDBW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `NOREN` reader - Nor flash access enable"]
pub type NOREN_R = crate::BitReader;
#[doc = "Field `NOREN` writer - Nor flash access enable"]
pub type NOREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCBEN` reader - Synchronous burst enable"]
pub type SYNCBEN_R = crate::BitReader;
#[doc = "Field `SYNCBEN` writer - Synchronous burst enable"]
pub type SYNCBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NWPOL` reader - NWAIT polarity"]
pub type NWPOL_R = crate::BitReader;
#[doc = "Field `NWPOL` writer - NWAIT polarity"]
pub type NWPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRAPEN` reader - Wrapped enable"]
pub type WRAPEN_R = crate::BitReader;
#[doc = "Field `WRAPEN` writer - Wrapped enable"]
pub type WRAPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NWTCFG` reader - Wait timing configuration"]
pub type NWTCFG_R = crate::BitReader;
#[doc = "Field `NWTCFG` writer - Wait timing configuration"]
pub type NWTCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WEN` reader - Write enable"]
pub type WEN_R = crate::BitReader;
#[doc = "Field `WEN` writer - Write enable"]
pub type WEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NWSEN` reader - NWAIT in synchronous transfer enable"]
pub type NWSEN_R = crate::BitReader;
#[doc = "Field `NWSEN` writer - NWAIT in synchronous transfer enable"]
pub type NWSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWTD` reader - Read-write timing different"]
pub type RWTD_R = crate::BitReader;
#[doc = "Field `RWTD` writer - Read-write timing different"]
pub type RWTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NWASEN` reader - NWAIT in asynchronous transfer enable"]
pub type NWASEN_R = crate::BitReader;
#[doc = "Field `NWASEN` writer - NWAIT in asynchronous transfer enable"]
pub type NWASEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRPGS` reader - CRAM page size"]
pub type CRPGS_R = crate::FieldReader;
#[doc = "Field `CRPGS` writer - CRAM page size"]
pub type CRPGS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MWMC` reader - Memory write mode control"]
pub type MWMC_R = crate::BitReader;
#[doc = "Field `MWMC` writer - Memory write mode control"]
pub type MWMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Memory bank enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address and data multiplexing enable"]
    #[inline(always)]
    pub fn admuxen(&self) -> ADMUXEN_R {
        ADMUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Memory device type"]
    #[inline(always)]
    pub fn dev(&self) -> DEV_R {
        DEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    pub fn extmdbw(&self) -> EXTMDBW_R {
        EXTMDBW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Nor flash access enable"]
    #[inline(always)]
    pub fn noren(&self) -> NOREN_R {
        NOREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronous burst enable"]
    #[inline(always)]
    pub fn syncben(&self) -> SYNCBEN_R {
        SYNCBEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NWAIT polarity"]
    #[inline(always)]
    pub fn nwpol(&self) -> NWPOL_R {
        NWPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wrapped enable"]
    #[inline(always)]
    pub fn wrapen(&self) -> WRAPEN_R {
        WRAPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wait timing configuration"]
    #[inline(always)]
    pub fn nwtcfg(&self) -> NWTCFG_R {
        NWTCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NWAIT in synchronous transfer enable"]
    #[inline(always)]
    pub fn nwsen(&self) -> NWSEN_R {
        NWSEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Read-write timing different"]
    #[inline(always)]
    pub fn rwtd(&self) -> RWTD_R {
        RWTD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NWAIT in asynchronous transfer enable"]
    #[inline(always)]
    pub fn nwasen(&self) -> NWASEN_R {
        NWASEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    pub fn crpgs(&self) -> CRPGS_R {
        CRPGS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Memory write mode control"]
    #[inline(always)]
    pub fn mwmc(&self) -> MWMC_R {
        MWMC_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK1CTRL3")
            .field("mwmc", &format_args!("{}", self.mwmc().bit()))
            .field("crpgs", &format_args!("{}", self.crpgs().bits()))
            .field("nwasen", &format_args!("{}", self.nwasen().bit()))
            .field("rwtd", &format_args!("{}", self.rwtd().bit()))
            .field("nwsen", &format_args!("{}", self.nwsen().bit()))
            .field("wen", &format_args!("{}", self.wen().bit()))
            .field("nwtcfg", &format_args!("{}", self.nwtcfg().bit()))
            .field("wrapen", &format_args!("{}", self.wrapen().bit()))
            .field("nwpol", &format_args!("{}", self.nwpol().bit()))
            .field("syncben", &format_args!("{}", self.syncben().bit()))
            .field("noren", &format_args!("{}", self.noren().bit()))
            .field("extmdbw", &format_args!("{}", self.extmdbw().bits()))
            .field("dev", &format_args!("{}", self.dev().bits()))
            .field("admuxen", &format_args!("{}", self.admuxen().bit()))
            .field("en", &format_args!("{}", self.en().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BK1CTRL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Memory bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<BK1CTRL3_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Address and data multiplexing enable"]
    #[inline(always)]
    #[must_use]
    pub fn admuxen(&mut self) -> ADMUXEN_W<BK1CTRL3_SPEC, 1> {
        ADMUXEN_W::new(self)
    }
    #[doc = "Bits 2:3 - Memory device type"]
    #[inline(always)]
    #[must_use]
    pub fn dev(&mut self) -> DEV_W<BK1CTRL3_SPEC, 2> {
        DEV_W::new(self)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn extmdbw(&mut self) -> EXTMDBW_W<BK1CTRL3_SPEC, 4> {
        EXTMDBW_W::new(self)
    }
    #[doc = "Bit 6 - Nor flash access enable"]
    #[inline(always)]
    #[must_use]
    pub fn noren(&mut self) -> NOREN_W<BK1CTRL3_SPEC, 6> {
        NOREN_W::new(self)
    }
    #[doc = "Bit 8 - Synchronous burst enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncben(&mut self) -> SYNCBEN_W<BK1CTRL3_SPEC, 8> {
        SYNCBEN_W::new(self)
    }
    #[doc = "Bit 9 - NWAIT polarity"]
    #[inline(always)]
    #[must_use]
    pub fn nwpol(&mut self) -> NWPOL_W<BK1CTRL3_SPEC, 9> {
        NWPOL_W::new(self)
    }
    #[doc = "Bit 10 - Wrapped enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrapen(&mut self) -> WRAPEN_W<BK1CTRL3_SPEC, 10> {
        WRAPEN_W::new(self)
    }
    #[doc = "Bit 11 - Wait timing configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nwtcfg(&mut self) -> NWTCFG_W<BK1CTRL3_SPEC, 11> {
        NWTCFG_W::new(self)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WEN_W<BK1CTRL3_SPEC, 12> {
        WEN_W::new(self)
    }
    #[doc = "Bit 13 - NWAIT in synchronous transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn nwsen(&mut self) -> NWSEN_W<BK1CTRL3_SPEC, 13> {
        NWSEN_W::new(self)
    }
    #[doc = "Bit 14 - Read-write timing different"]
    #[inline(always)]
    #[must_use]
    pub fn rwtd(&mut self) -> RWTD_W<BK1CTRL3_SPEC, 14> {
        RWTD_W::new(self)
    }
    #[doc = "Bit 15 - NWAIT in asynchronous transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn nwasen(&mut self) -> NWASEN_W<BK1CTRL3_SPEC, 15> {
        NWASEN_W::new(self)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    #[must_use]
    pub fn crpgs(&mut self) -> CRPGS_W<BK1CTRL3_SPEC, 16> {
        CRPGS_W::new(self)
    }
    #[doc = "Bit 19 - Memory write mode control"]
    #[inline(always)]
    #[must_use]
    pub fn mwmc(&mut self) -> MWMC_W<BK1CTRL3_SPEC, 19> {
        MWMC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SRAM/NOR-Flash chip-select control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK1CTRL3_SPEC;
impl crate::RegisterSpec for BK1CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk1ctrl3::R`](R) reader structure"]
impl crate::Readable for BK1CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk1ctrl3::W`](W) writer structure"]
impl crate::Writable for BK1CTRL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK1CTRL3 to value 0x30db"]
impl crate::Resettable for BK1CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x30db;
}
