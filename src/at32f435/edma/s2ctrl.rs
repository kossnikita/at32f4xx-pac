#[doc = "Register `S2CTRL` reader"]
pub type R = crate::R<S2CTRL_SPEC>;
#[doc = "Register `S2CTRL` writer"]
pub type W = crate::W<S2CTRL_SPEC>;
#[doc = "Field `SEN` reader - Stream enable / flag stream ready when read low"]
pub type SEN_R = crate::BitReader;
#[doc = "Field `SEN` writer - Stream enable / flag stream ready when read low"]
pub type SEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRIEN` reader - Direct mode error interrupt enable"]
pub type DMERRIEN_R = crate::BitReader;
#[doc = "Field `DMERRIEN` writer - Direct mode error interrupt enable"]
pub type DMERRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRIEN` reader - Transfer error interrupt enable"]
pub type DTERRIEN_R = crate::BitReader;
#[doc = "Field `DTERRIEN` writer - Transfer error interrupt enable"]
pub type DTERRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTIEN` reader - Half data transfer interrupt enable"]
pub type HDTIEN_R = crate::BitReader;
#[doc = "Field `HDTIEN` writer - Half data transfer interrupt enable"]
pub type HDTIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTIEN` reader - Full data transfer complete interrupt enable"]
pub type FDTIEN_R = crate::BitReader;
#[doc = "Field `FDTIEN` writer - Full data transfer complete interrupt enable"]
pub type FDTIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFCTRL` reader - Peripheral flow controller"]
pub type PFCTRL_R = crate::BitReader;
#[doc = "Field `PFCTRL` writer - Peripheral flow controller"]
pub type PFCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTD` reader - Data transfer direction"]
pub type DTD_R = crate::FieldReader;
#[doc = "Field `DTD` writer - Data transfer direction"]
pub type DTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LM` reader - Loop mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loop mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINCM` reader - Peripheral increment mode"]
pub type PINCM_R = crate::BitReader;
#[doc = "Field `PINCM` writer - Peripheral increment mode"]
pub type PINCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINCM` reader - Memory increment mode"]
pub type MINCM_R = crate::BitReader;
#[doc = "Field `MINCM` writer - Memory increment mode"]
pub type MINCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWIDTH` reader - Peripheral data width"]
pub type PWIDTH_R = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Peripheral data width"]
pub type PWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWIDTH` reader - Memory data width"]
pub type MWIDTH_R = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Memory data width"]
pub type MWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PINCOS` reader - Peripheral increment offset size"]
pub type PINCOS_R = crate::BitReader;
#[doc = "Field `PINCOS` writer - Peripheral increment offset size"]
pub type PINCOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPL` reader - Stream priority level"]
pub type SPL_R = crate::FieldReader;
#[doc = "Field `SPL` writer - Stream priority level"]
pub type SPL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMM` reader - Double memory mode"]
pub type DMM_R = crate::BitReader;
#[doc = "Field `DMM` writer - Double memory mode"]
pub type DMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM` reader - Current memory (only in double buffer mode)"]
pub type CM_R = crate::BitReader;
#[doc = "Field `CM` writer - Current memory (only in double buffer mode)"]
pub type CM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBURST` reader - Peripheral burst transmission"]
pub type PBURST_R = crate::FieldReader;
#[doc = "Field `PBURST` writer - Peripheral burst transmission"]
pub type PBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MBURST` reader - Memory burst transmission"]
pub type MBURST_R = crate::FieldReader;
#[doc = "Field `MBURST` writer - Memory burst transmission"]
pub type MBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmerrien(&self) -> DMERRIEN_R {
        DMERRIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn dterrien(&self) -> DTERRIEN_R {
        DTERRIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half data transfer interrupt enable"]
    #[inline(always)]
    pub fn hdtien(&self) -> HDTIEN_R {
        HDTIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Full data transfer complete interrupt enable"]
    #[inline(always)]
    pub fn fdtien(&self) -> FDTIEN_R {
        FDTIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dtd(&self) -> DTD_R {
        DTD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Loop mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pincm(&self) -> PINCM_R {
        PINCM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn mincm(&self) -> MINCM_R {
        MINCM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Peripheral data width"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Memory data width"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Stream priority level"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Double memory mode"]
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Current memory (only in double buffer mode)"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Peripheral burst transmission"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Memory burst transmission"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2CTRL")
            .field("mburst", &format_args!("{}", self.mburst().bits()))
            .field("pburst", &format_args!("{}", self.pburst().bits()))
            .field("cm", &format_args!("{}", self.cm().bit()))
            .field("dmm", &format_args!("{}", self.dmm().bit()))
            .field("spl", &format_args!("{}", self.spl().bits()))
            .field("pincos", &format_args!("{}", self.pincos().bit()))
            .field("mwidth", &format_args!("{}", self.mwidth().bits()))
            .field("pwidth", &format_args!("{}", self.pwidth().bits()))
            .field("mincm", &format_args!("{}", self.mincm().bit()))
            .field("pincm", &format_args!("{}", self.pincm().bit()))
            .field("lm", &format_args!("{}", self.lm().bit()))
            .field("dtd", &format_args!("{}", self.dtd().bits()))
            .field("pfctrl", &format_args!("{}", self.pfctrl().bit()))
            .field("fdtien", &format_args!("{}", self.fdtien().bit()))
            .field("hdtien", &format_args!("{}", self.hdtien().bit()))
            .field("dterrien", &format_args!("{}", self.dterrien().bit()))
            .field("dmerrien", &format_args!("{}", self.dmerrien().bit()))
            .field("sen", &format_args!("{}", self.sen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S2CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SEN_W<S2CTRL_SPEC> {
        SEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrien(&mut self) -> DMERRIEN_W<S2CTRL_SPEC> {
        DMERRIEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterrien(&mut self) -> DTERRIEN_W<S2CTRL_SPEC> {
        DTERRIEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Half data transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdtien(&mut self) -> HDTIEN_W<S2CTRL_SPEC> {
        HDTIEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Full data transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdtien(&mut self) -> FDTIEN_W<S2CTRL_SPEC> {
        FDTIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn pfctrl(&mut self) -> PFCTRL_W<S2CTRL_SPEC> {
        PFCTRL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DTD_W<S2CTRL_SPEC> {
        DTD_W::new(self, 6)
    }
    #[doc = "Bit 8 - Loop mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<S2CTRL_SPEC> {
        LM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pincm(&mut self) -> PINCM_W<S2CTRL_SPEC> {
        PINCM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn mincm(&mut self) -> MINCM_W<S2CTRL_SPEC> {
        MINCM_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Peripheral data width"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<S2CTRL_SPEC> {
        PWIDTH_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - Memory data width"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MWIDTH_W<S2CTRL_SPEC> {
        MWIDTH_W::new(self, 13)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    #[must_use]
    pub fn pincos(&mut self) -> PINCOS_W<S2CTRL_SPEC> {
        PINCOS_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Stream priority level"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<S2CTRL_SPEC> {
        SPL_W::new(self, 16)
    }
    #[doc = "Bit 18 - Double memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmm(&mut self) -> DMM_W<S2CTRL_SPEC> {
        DMM_W::new(self, 18)
    }
    #[doc = "Bit 19 - Current memory (only in double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<S2CTRL_SPEC> {
        CM_W::new(self, 19)
    }
    #[doc = "Bits 21:22 - Peripheral burst transmission"]
    #[inline(always)]
    #[must_use]
    pub fn pburst(&mut self) -> PBURST_W<S2CTRL_SPEC> {
        PBURST_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Memory burst transmission"]
    #[inline(always)]
    #[must_use]
    pub fn mburst(&mut self) -> MBURST_W<S2CTRL_SPEC> {
        MBURST_W::new(self, 23)
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
#[doc = "stream 2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2CTRL_SPEC;
impl crate::RegisterSpec for S2CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2ctrl::R`](R) reader structure"]
impl crate::Readable for S2CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s2ctrl::W`](W) writer structure"]
impl crate::Writable for S2CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S2CTRL to value 0"]
impl crate::Resettable for S2CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
